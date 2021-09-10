from tensorflow/tensorflow:latest

RUN echo "hey"

COPY . /app

WORKDIR app

RUN ls


RUN pip install waitress
RUN pip install flask
RUN pip install flask_cors


CMD python3 server.py