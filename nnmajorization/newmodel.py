

import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers


import json
import os
import numpy as np




inputs = keras.Input(shape=(1899))
m1 = layers.Dense(500, activation="relu") (inputs)


m1 = layers.Dense(500, activation="relu") (m1)
m1 = layers.Dense(500, activation="relu") (m1)
m1 = layers.Dense(500, activation="relu") (m1)
m1 = layers.Dense(500, activation="relu") (m1)
m1 = layers.Dense(500, activation="relu") (m1)
m1 = layers.Dense(500, activation="relu") (m1)

o = layers.Dense(500, activation="relu") (m1)
outputs = layers.Dense( 2 )(o)


model = keras.Model(inputs=inputs, outputs=outputs, name="majormodel")



model.save('MODEL')