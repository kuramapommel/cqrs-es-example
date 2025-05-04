lazy val akkaHttpVersion = "10.7.0"
lazy val akkaVersion = "2.10.3"
lazy val akkaManagementVersion = "1.6.0"

resolvers += "Akka library repository".at("https://repo.akka.io/maven")

// Run in a separate JVM, to make sure sbt waits until all threads have
// finished before returning.
// If you want to keep the application running while executing other
// sbt tasks, consider https://github.com/spray/sbt-revolver/
fork := true

// module は Java SE 9 以降で導入されたものであり、Java SE 8 準拠の場合は認識しないため破棄（discard）してしまって良い
// 破棄しなければ deduplicate エラーを起こす
// 参照： https://qiita.com/yokra9/items/1e72646623f962ce02ee
assembly / assemblyMergeStrategy := {
  case PathList(ps @ _*) if ps.last endsWith "module-info.class" =>
    MergeStrategy.discard
  case PathList("META-INF", xs @ _*) => MergeStrategy.discard // META-INF内のファイルを無視
  case x =>
    val oldStrategy = (assembly / assemblyMergeStrategy).value
    oldStrategy(x)
}
assembly / assemblyJarName := "cqrs-es-example-write-api-server.jar" // jar ファイル名を指定

lazy val root = (project in file(".")).settings(
  inThisBuild(
    List(
      organization := "com.kuramapommel",
      scalaVersion := "3.6.4",
      semanticdbEnabled := true, // enable SemanticDB
      semanticdbVersion := scalafixSemanticdb.revision // only required for Scala 2.x
    )
  ),
  scalacOptions += {
    if (scalaVersion.value.startsWith("2.12"))
      "-Ywarn-unused-import"
    else
      "-Wunused:imports"
  },
  name := "write-api-server",
  libraryDependencies ++= Seq(
    "com.typesafe.akka" %% "akka-http" % akkaHttpVersion,
    "com.typesafe.akka" %% "akka-http-spray-json" % akkaHttpVersion,
    "com.typesafe.akka" %% "akka-actor-typed" % akkaVersion,
    "com.typesafe.akka" %% "akka-persistence-typed" % akkaVersion,
    "com.typesafe.akka" %% "akka-cluster-sharding-typed" % akkaVersion,
    "com.typesafe.akka" %% "akka-stream" % akkaVersion,
    "com.typesafe.akka" %% "akka-pki" % akkaVersion,
    "com.typesafe.akka" %% "akka-discovery" % akkaVersion,
    "com.typesafe.akka" %% "akka-serialization-jackson" % akkaVersion,
    "com.lightbend.akka" %% "akka-persistence-dynamodb" % "2.0.5",
    "com.lightbend.akka.management" %% "akka-management" % akkaManagementVersion,
    "com.lightbend.akka.management" %% "akka-management-cluster-bootstrap" % akkaManagementVersion,
    "com.lightbend.akka.discovery" %% "akka-discovery-kubernetes-api" % akkaManagementVersion,
    "ch.qos.logback" % "logback-classic" % "1.5.17",
    "commons-io" % "commons-io" % "2.19.0",
    "org.slf4j" % "slf4j-simple" % "1.7.36",

    // test dependencies
    "com.typesafe.akka" %% "akka-http-testkit" % akkaHttpVersion % Test,
    "com.typesafe.akka" %% "akka-actor-testkit-typed" % akkaVersion % Test,
    "org.scalatest" %% "scalatest" % "3.2.12" % Test
  ),
  dependencyOverrides += "org.slf4j" % "slf4j-api" % "1.7.36"
)
