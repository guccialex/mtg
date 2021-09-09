from tensorflow/tensorflow:latest

COPY . .

#RUN pip3 install tensorflow
#RUN pip3 install numpy

#RUN pip3 install autokeras


#RUN curl -sSL https://sdk.cloud.google.com | bash
RUN curl -O https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-sdk-356.0.0-linux-x86_64.tar.gz

RUN ./google-cloud-sdk/install.sh


RUN gsutil -m cp -r "gs://mtg-ml/PREDICTION/"  .


CMD python3 server.py