package main

/*
#cgo LDFLAGS: -lprocps
#include <proc/sysinfo.h>
*/
import "C"

type Uptime struct {
	Active float64 `json:"active"`
	Idle   float64 `json:"idle"`
}

func GetUptime() Uptime {
	var active C.double
	var idle C.double

	C.uptime(&active, &idle)

	return Uptime{
		Active: float64(active),
		Idle:   float64(idle),
	}
}
