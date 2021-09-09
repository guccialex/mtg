import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers


import json
import os
import numpy as np



import keras.losses


def custom_loss(y_true, y_pred):
    squared_difference = tf.square(y_true - y_pred)
    return tf.reduce_mean(squared_difference, axis=-1)

keras.losses.custom_loss = custom_loss




model = keras.models.load_model('MODEL')




fobj = open("standardalphabet.json", "rb")
data = json.load(fobj)
nparray = np.array(data)


nptest = np.zeros( (1,1899) )

#nptest[0][1883] = 1.0
nptest[0][52] = 1.0
#nptest[0][447] = 1.0
#nptest[0][176] = 1.0

prediction = model.predict(nptest)[0]



count = 0

for w in sorted( range(len(prediction)), key=lambda k: prediction[k] ):

    count +=1

    if count > 1820:
        print( nparray[w] )







'''

#got from here, useful
#https://stackoverflow.com/questions/52800025/keras-give-input-to-intermediate-layer-and-get-final-output


idx = 4  # index of desired layer
layer_input = keras.Input(shape=2) # a new input tensor to be able to feed the desired layer

# create the new nodes for each layer in the path
x = layer_input
for layer in model.layers[idx:]:
    x = layer(x)
layers.Dense(100, activation="relu") (inputs)
# create the model
model2 = keras.Model(layer_input, x)




nptest = np.array([[0.4,0.9]])

prediction = model2.predict(nptest)[0]

print(prediction)

fobj = open("standardalphabet.json", "rb")
data = json.load(fobj)
nparray = np.array(data)




#carddict = {}


count = 0

for x in prediction:

    #carddict[nparray[count]] = {x}

    if x > 0.20:
        print(nparray[count])
        print(x)

    count +=1



for w in sorted( range(len(prediction)), key=lambda k: prediction[k] ):

    print( nparray[w] )

'''



