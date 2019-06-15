package envmap

import (
	"testing"

	. "github.com/smartystreets/goconvey/convey"
)

func TestEnvMap(t *testing.T) {
	Convey("Test EnvMap", t, func() {
		Convey("SplitKeyValue", func() {
			var kv, k, v string
			kv = "FOO="
			k, v = SplitKeyValue(kv)
			So(k, ShouldEqual, "FOO")
			So(v, ShouldEqual, "")

			kv = "FOO=BAR"
			k, v = SplitKeyValue(kv)
			So(k, ShouldEqual, "FOO")
			So(v, ShouldEqual, "BAR")

			kv = "FOO=BAR=BAZ"
			k, v = SplitKeyValue(kv)
			So(k, ShouldEqual, "FOO")
			So(v, ShouldEqual, "BAR=BAZ")
		})
		Convey("MergeKeyValue", func() {
			var kv, k, v string
			k = "FOO"
			v = ""
			kv = MergeKeyValue(k, v)
			So(kv, ShouldEqual, "FOO=")

			k = "FOO"
			v = "BAR"
			kv = MergeKeyValue(k, v)
			So(kv, ShouldEqual, "FOO=BAR")

			k = "FOO"
			v = "BAR=BAZ"
			kv = MergeKeyValue(k, v)
			So(kv, ShouldEqual, "FOO=BAR=BAZ")
		})

		Convey("New then Environ", func() {
			em := New([]string{"FOO=BAR"})
			result := em.Environ()
			So(result, ShouldContain, "FOO=BAR")
		})

		Convey("Change env", func() {
			em := New([]string{"FOO=BAR", "A=B"})
			em.Set("FOO", "BAZ")
			result := em.Environ()
			So(result, ShouldContain, "FOO=BAZ")
			So(result, ShouldContain, "A=B")
		})

		Convey("Add env", func() {
			em := New([]string{"FOO=BAR", "A=B"})
			em.Set("C", "D")
			em.Set("E", "")
			result := em.Environ()
			So(result, ShouldContain, "FOO=BAR")
			So(result, ShouldContain, "A=B")
			So(result, ShouldContain, "C=D")
			So(result, ShouldContain, "E=")
		})

		Convey("Unset", func() {
			em := New([]string{"FOO=BAR", "A=B"})
			em.Unset("FOO")
			result := em.Environ()
			So(len(result), ShouldEqual, 1)
			So(result[0], ShouldEqual, "A=B")
		})
	})

}
