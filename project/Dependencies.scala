import sbt._

object Dependencies {
  lazy val guice = "com.google.inject" % "guice" % "4.2.3"
  lazy val scalaGuice = "net.codingwell" %% "scala-guice" % "4.2.6"
  lazy val jackson = "com.fasterxml.jackson.module" %% "jackson-module-scala" % "2.10.3"
}
