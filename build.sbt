val scala3Version = "3.0.0"
ThisBuild / organization := "dev.ligature"

lazy val root = project
  .in(file("."))
  .settings(
    name := "raccoon",
    version := "0.1.0-SNAPSHOT",
    scalaVersion := scala3Version,
    libraryDependencies += "com.novocode" % "junit-interface" % "0.11" % "test"
  )
