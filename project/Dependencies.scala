import sbt._

object Dependencies {
  lazy val guice = "com.google.inject" % "guice" % "4.2.2"
  lazy val scalaGuice = "net.codingwell" %% "scala-guice" % "4.2.2"
  lazy val jackson = "com.fasterxml.jackson.module" %% "jackson-module-scala" % "2.9.6"
}
