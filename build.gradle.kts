plugins {
    id("org.jetbrains.kotlin.jvm") version "1.4.20"
    `java-library`
    `maven-publish`
}

repositories {
    mavenLocal()
    mavenCentral()
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "dev.ligature"
            artifactId = "rakkoon"
            version = "0.1.0-SNAPSHOT"

            from(components["java"])
        }
    }
}

dependencies {
    implementation(platform("org.jetbrains.kotlin:kotlin-bom"))
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")
    api("io.arrow-kt:arrow-core:0.13.1")
    testImplementation("io.kotest:kotest-runner-junit5:4.4.3")
    testImplementation("io.kotest:kotest-assertions-core:4.4.3")
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions {
        freeCompilerArgs = freeCompilerArgs + "-Xallow-result-return-type"
        jvmTarget = "1.8"
    }
}

tasks.withType<Test> {
    useJUnitPlatform()
}
