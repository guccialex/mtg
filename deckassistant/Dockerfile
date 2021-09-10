from tensorflow/tensorflow:latest

COPY . .

#RUN pip3 install tensorflow
#RUN pip3 install numpy
RUN pip install waitress
RUN pip install flask
RUN pip install flask_cors


CMD waitress-serve --port=8000 server:app