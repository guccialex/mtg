import numpy as np
import tensorflow as tf

import autokeras as ak

from tensorflow import keras
from tensorflow.keras import layers

import json
import os
import numpy as np

import mtgmodule



fobj = open("cardstolands.json", "rb")
sets = json.load(fobj)

cardsets = sets[0]
landsets = sets[1]

train_inputs = []
train_labels = []



for cards in cardsets:
    data = mtgmodule.cardstoonehot(cards)
    train_inputs.append( data )

for lands in landsets:
    data = mtgmodule.cardstoonehot(lands)
    train_labels.append( data )
    


train_inputs = np.array( train_inputs )
train_labels = np.array( train_labels )



input_node = ak.Input()
output_node = ak.DenseBlock() (input_node)
output_node = ak.RegressionHead() (output_node)


auto_model = ak.AutoModel(input_node, output_node, overwrite=True, max_trials=20)


auto_model.fit(train_inputs, train_labels, epochs=4)

model = auto_model.export_model()


model.fit(train_inputs, train_labels, batch_size=100, epochs=30)

#model.save( "LANDS")



try:
    model.save("LANDS", save_format="tf")
except Exception:
    model.save("LANDS.h5")

'''

import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers

import json
import os
import numpy as np

import mtgmodule



fobj = open("cardstolands.json", "rb")
sets = json.load(fobj)

cardsets = sets[0]
landsets = sets[1]

train_inputs = []
train_labels = []

for cards in cardsets:
    data = mtgmodule.cardstoonehot(cards)
    train_inputs.append( data )

for lands in landsets:
    data = mtgmodule.cardstoonehot(lands)
    train_labels.append( data )


train_inputs = np.array( train_inputs )
train_labels = np.array( train_labels )




standardlength = mtgmodule.standardlength()


inputs = keras.Input( standardlength )
m1 = layers.Dense(2000, activation="relu") (inputs)

m1 = layers.Dense(1000, activation="relu") (m1)
m1 = layers.Dense(100, activation="relu") (m1)
m1 = layers.Dense(20, activation="relu") (m1)
m1 = layers.Dense(20, activation="relu") (m1)
m1 = layers.Dense(100, activation="relu") (m1)
m1 = layers.Dense(1000, activation="relu") (m1)

o = layers.Dense(2000, activation="relu") (m1)
outputs = layers.Dense( standardlength )(o)


model = keras.Model(inputs=inputs, outputs=outputs, name="majormodel")



model.compile(
    loss= keras.losses.MeanSquaredError(),
    optimizer=tf.keras.optimizers.Adam(),
    metrics=["accuracy"],
)

model.fit(train_inputs, train_labels, batch_size=100, epochs=25)

model.save('LANDS')

'''