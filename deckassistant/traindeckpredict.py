import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers

import json
import os
import numpy as np

import mtgmodule


train_inputs = []

fobj = open("decks.json", "rb")
decks = json.load(fobj)

for deck in decks:
    data = mtgmodule.cardstoonehot(deck)
    train_inputs.append( data )


train_inputs = np.array( train_inputs )
train_labels = train_inputs




standardlength = mtgmodule.standardlength()


inputs = keras.Input( standardlength )
m1 = layers.Dense(200, activation="relu") (inputs)

m1 = layers.Dense(100, activation="relu") (m1)
m1 = layers.Dense(100, activation="relu") (m1)

o = layers.Dense(200, activation="relu") (m1)
outputs = layers.Dense( standardlength )(o)


model = keras.Model(inputs=inputs, outputs=outputs, name="majormodel")



model.compile(
    loss= keras.losses.MeanSquaredError(),
    optimizer=tf.keras.optimizers.Adam(),
    metrics=["accuracy"],
)

model.fit(train_inputs, train_labels, batch_size=10, epochs=4)

model.save('PREDICTION')

