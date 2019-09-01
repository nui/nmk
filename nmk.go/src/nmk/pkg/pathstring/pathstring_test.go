package pathstring

import (
	"os"
	"strings"
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestPathString(t *testing.T) {
	Convey("Test PathString", t, func() {
		foo := "/foo"
		bar := "/bar"
		//baz := "/baz"
		Convey("empty path", func() {
			Convey("Make() should return empty string", func() {
				sp := Parse("")
				So(sp.Make(), ShouldEqual, "")
			})
			Convey("Prepend one item should return as is", func() {
				sp := Parse("")
				sp.Prepend(foo)
				So(sp.Make(), ShouldEqual, foo)
			})
			Convey("Append one item should return as is", func() {
				sp := Parse("")
				sp.Append(foo)
				So(sp.Make(), ShouldEqual, foo)
			})
			Convey("Prepend two items should return correct order", func() {
				sp := Parse("")
				sp.Prepend(foo).Prepend(bar)
				expect := bar + string(os.PathListSeparator) + foo
				So(sp.Make(), ShouldEqual, expect)
			})
		})
		Convey("non empty path", func() {
			Convey("with 2 items", func() {
				parts := []string{foo, bar}
				joinedPath := strings.Join(parts, string(os.PathListSeparator))
				Convey("Prepend existing should not swap order", func() {
					sp := Parse(joinedPath)
					sp.Prepend(foo)
					So(sp.Make(), ShouldEqual, joinedPath)
				})
				Convey("Prepend existing should swap order", func() {
					sp := Parse(joinedPath)
					sp.Prepend(bar)
					expected := strings.Join([]string{bar, foo}, string(os.PathListSeparator))
					So(sp.Make(), ShouldEqual, expected)
				})
			})
		})
	})
}
