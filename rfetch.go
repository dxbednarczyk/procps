package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"net/http"
	"time"

	"github.com/akyoto/cache"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

var routes = []string{"/", "/ping", "/meminfo", "/loadinfo", "/uptime", "/cpuinfo"}

type InfoType interface {
	MemInfo | LoadInfo | Uptime | CPUInfo
}

func main() {
	c := cache.New(5 * time.Second)
	e := echo.New()

	e.Use(middleware.Logger())
	e.Use(middleware.Recover())

	e.GET("/", func(c echo.Context) error {
		m, err := json.Marshal(routes)
		if err != nil {
			return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
		}

		return c.JSON(http.StatusOK, m)
	})

	e.GET("/ping", func(c echo.Context) error {
		return c.JSON(http.StatusOK, []byte(`{"response": "pong"}`))
	})

	e.GET("/meminfo", cachedJsonRoute("meminfo", GetMemInfo, c, 1*time.Second))
	e.GET("/loadinfo", cachedJsonRoute("loadinfo", GetLoadInfo, c, 1*time.Minute))
	e.GET("/uptime", cachedJsonRoute("uptime", GetUptime, c, 1*time.Second))

	// I doubt this program will still be running after 200+ years!
	e.GET("/cpuinfo", cachedJsonRoute("cpuinfo", GetCPUInfo, c, time.Duration(1<<63-1)))

	port := flag.Int("p", 8080, "port to bind to")
	flag.Parse()

	e.Logger.Fatal(e.Start(fmt.Sprintf(":%d", *port)))
}

func cachedJsonRoute[T InfoType](name string, tocall func() T, cache *cache.Cache, duration time.Duration) func(c echo.Context) error {
	return func(c echo.Context) error {
		var tw []byte

		cached, found := cache.Get(name)

		if !found {
			m, err := json.Marshal(tocall())

			if err != nil {
				return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
			}

			tw = m
			cache.Set(name, m, duration)
		} else {
			tw = cached.([]byte)
		}

		return c.JSON(http.StatusOK, tw)
	}
}
