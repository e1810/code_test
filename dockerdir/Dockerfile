FROM debian:10.4
FROM gcc:9.2
WORKDIR /code_executer

COPY ./Main.sh ./Main.sh
COPY ./Main.c ./Main.c
COPY ./Main.cpp ./Main.cpp
COPY ./Main.rs ./Main.rs


RUN gcc Main.c -O2 -o Main_c.out -lm
RUN g++ Main.cpp -o Main_cpp.out

CMD ["echo", ":("]
