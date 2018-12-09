package main

import (
	"container/ring"
	"fmt"
	"strconv"
	"strings"

	"github.com/aqatl/fileutils"
)

func main() {
	input := strings.Split(fileutils.MustLoadToString("input.txt"), " ")
	players, _ := strconv.ParseUint(input[0], 10, 64)
	marbles, _ := strconv.ParseUint(input[6], 10, 64)

	fmt.Println("Part 1:", part1(players, marbles))
	fmt.Println("Part 2:", part1(players, marbles*100))
}

func part1(players, marbles uint64) uint64 {
	scores := make([]uint64, players)
	playerIdx := uint64(0)

	r := &ring.Ring{Value: 0}
	for i := uint64(1); i <= marbles; i++ {
		if i%23 == 0 {
			for j := 0; j < 8; j++ {
				r = r.Prev()
			}
			scores[playerIdx] += r.Unlink(1).Value.(uint64) + i
			r = r.Next()
		} else {
			r = r.Next()
			r.Link(&ring.Ring{Value: i})
			r = r.Next()
		}
		playerIdx = uint64(playerIdx+1) % players
	}

	max := uint64(0)
	for _, v := range scores {
		if v > max {
			max = v
		}
	}
	return max
}
