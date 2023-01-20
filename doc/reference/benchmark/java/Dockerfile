FROM debian:bullseye

RUN apt update && apt -y install openjdk-17-jdk

COPY . .

RUN javac main.java

CMD ["java", "MyServer"]
