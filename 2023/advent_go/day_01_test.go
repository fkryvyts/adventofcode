package adventofcode

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"testing"
	"unicode/utf8"
)

func TestDay01(t *testing.T) {
	b, err := os.ReadFile("inputs/day_01.txt")
	if err != nil {
		t.Fatal(err)
	}

	digitWords := []string{
		"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	}

	reversedDigitWords := reverseAll(digitWords)

	lines := strings.Split(string(b), "\n")

	sum := 0

	for _, line := range lines {
		first := parseDigits(line, digitWords)
		last := parseDigits(reverse(line), reversedDigitWords)

		sum += first*10 + last
	}

	fmt.Println(sum)
}

func parseDigits(line string, words []string) int {
	buff := ""

	for _, c := range line {
		cs := string(c)

		d, err := strconv.Atoi(cs)
		if err == nil {
			return d
		}

		buff += cs

		for i, word := range words {
			if strings.HasSuffix(buff, word) {
				return i + 1
			}
		}
	}

	return 0
}

func reverseAll(ss []string) []string {
	res := make([]string, len(ss))

	for i, s := range ss {
		res[i] = reverse(s)
	}

	return res
}

func reverse(s string) string {
	l := utf8.RuneCountInString(s)
	res := make([]rune, l)

	for i, c := range s {
		res[l-i-1] = c
	}

	return string(res)
}
