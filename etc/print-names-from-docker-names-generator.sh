#!/bin/sh

concat() {
    wget -qO- https://raw.githubusercontent.com/docker/docker/master/pkg/namesgenerator/names-generator.go \
        | sed 's/package namesgenerator/package main/g' \
        | sed '/import (/a \\t"sort"'
    cat << 'EOCODE'
func main() {
	fmt.Printf("left = {\n")
	sort.Strings(left[:])
	for _, value := range left {
		fmt.Printf("\"%s\",\n", value)
	}
	fmt.Printf("}\n\n")
	fmt.Printf("right = {\n")
	sort.Strings(right[:])
	for _, value := range right {
		fmt.Printf("\"%s\",\n", value)
	}
	fmt.Printf("}")
}
EOCODE
}
go_file=$(tempfile --suffix=.go)
concat > $go_file
go run $go_file
rm $go_file
