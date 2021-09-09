from python

COPY . .

RUN pip3 install tensorflow
RUN pip3 install numpy
RUN pip3 install autokeras


RUN gsutil -m cp -r "gs://mtg-ml/PREDICTION/"  .


CMD python3 server.py