package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/akyoto/cache"
	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
)

var routes = []string{"/", "/ping", "/meminfo", "/loadinfo", "/uptime", "/cpuinfo"}

type InfoType interface {
	MemInfo | LoadInfo | Uptime | CPUInfo
}

func main() {
	port := flag.Int("p", 8080, "port to bind to")

	c := cache.New(5 * time.Second)
	r := chi.NewRouter()

	r.Use(middleware.Logger)

	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		m, err := json.Marshal(routes)
		if err != nil {
			w.WriteHeader(500)
			w.Write([]byte("Could not marshal data"))
			return
		}

		w.Header().Set("Content-Type", "application/json")
		w.Write(m)
	})

	r.Get("/ping", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte("{\"response\": \"pong\"}"))
	})

	r.Get("/meminfo", cachedJsonRoute("meminfo", GetMemInfo, c, 1*time.Second))
	r.Get("/loadinfo", cachedJsonRoute("loadinfo", GetLoadInfo, c, 1*time.Minute))
	r.Get("/uptime", cachedJsonRoute("uptime", GetUptime, c, 1*time.Second))

	// I doubt this program will still be running while you change the CPU!
	r.Get("/cpuinfo", cachedJsonRoute("cpuinfo", GetCPUInfo, c, time.Duration(1<<63-1)))

	flag.Parse()

	log.Printf("Listening on :%d\n", *port)
	http.ListenAndServe(fmt.Sprintf(":%d", *port), r)
}

func cachedJsonRoute[T InfoType](name string, tocall func() T, cache *cache.Cache, duration time.Duration) func(http.ResponseWriter, *http.Request) {
	return func(w http.ResponseWriter, r *http.Request) {
		var tw []byte

		cached, found := cache.Get(name)

		if !found {
			m, err := json.Marshal(tocall())

			if err != nil {
				w.WriteHeader(500)
				w.Write([]byte("Could not marshal data"))
				return
			}

			tw = m
			cache.Set(name, m, duration)
		} else {
			tw = cached.([]byte)
		}

		w.Header().Add("Content-Type", "application/json")
		w.Write(tw)
	}
}
