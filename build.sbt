import Dependencies._

lazy val root = (project in file(".")).
  settings(
    inThisBuild(List(
      organization := "com.nuimk",
      scalaVersion := "2.13.3",
      version := "0.0.1"
    )),
    name := "nmk",
    libraryDependencies ++= Seq(
      scalaGuice,
      jackson,
    )
  )

scalacOptions += "-deprecation"
