package main

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"

	_ "modernc.org/sqlite"
)

func main() {
	r := chi.NewRouter()

	// A good base middleware stack
	r.Use(middleware.RequestID)
	r.Use(middleware.RealIP)
	r.Use(middleware.Logger)
	r.Use(middleware.Recoverer)

	// Set a timeout value on the request context (ctx), that will signal
	// through ctx.Done() that the request has timed out and further
	// processing should be stopped.
	r.Use(middleware.Timeout(60 * time.Second))

	r.Use(DbCtx)

	r.Get("/", getHello)

	http.ListenAndServe(":8000", r)
}

func DbCtx(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		db, err := sql.Open("sqlite", "file:aino.db")
		if err != nil {
			log.Print(err)
			http.Error(w, http.StatusText(500), 500)
			return
		}
		ctx := context.WithValue(r.Context(), "db", db)
		next.ServeHTTP(w, r.WithContext(ctx))
	})
}

func getHello(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	db, ok := ctx.Value("db").(*sql.DB)
	if !ok {
		log.Print("failed to get database structs from context")
		http.Error(w, http.StatusText(500), 500)
		return
	}
	rows, err := db.QueryContext(ctx, "SELECT 10;")
	if err != nil {
		log.Print(err)
		http.Error(w, http.StatusText(500), 500)
		return
	}
	defer rows.Close()

	var result int64
	rows.Next()
	rows.Scan(&result)
	w.Write([]byte(fmt.Sprintf("number:%d", result)))

	if err := rows.Err(); err != nil {
		log.Fatal(err)
	}
}
