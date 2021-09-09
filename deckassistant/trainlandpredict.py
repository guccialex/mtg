import numpy as np
import tensorflow as tf

from tensorflow import keras
from tensorflow.keras import layers

import json
import os
import numpy as np

import mtgmodule
import keras_tuner as kt




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
m1 = layers.Dense(1000, activation="relu") (m1)
m1 = layers.Dense(1000, activation="relu") (m1)
m1 = layers.Dense(1000, activation="relu") (m1)
o = layers.Dense(2000, activation="relu") (m1)
outputs = layers.Dense( standardlength )(o)
model = keras.Model(inputs=inputs, outputs=outputs, name="majormodel")
model.compile(
    loss= keras.losses.MeanSquaredError(),
    optimizer=tf.keras.optimizers.Adam(),
    metrics=["accuracy"],
)

model.fit(train_inputs, train_labels, batch_size=20, epochs=3)
model.save('LANDS')

'''

def model_builder(hp):


    inputs = keras.Input( standardlength )

    m1 = layers.Dense(2000, activation="relu") (inputs)
    
    m1 = layers.Dense(1000, activation="relu") (m1)
    m1 = layers.Dense(1000, activation="relu") (m1)

    o = layers.Dense(2000, activation="relu") (m1)

    outputs = layers.Dense( standardlength )(o)

    
    model = keras.Model(inputs=inputs, outputs=outputs, name="majormodel")

    hp_learning_rate = hp.Choice('learning_rate', values=[1e-2, 1e-3, 1e-4])

    model.compile(
        loss=keras.losses.MeanSquaredError(),
        optimizer=keras.optimizers.Adam(learning_rate=hp_learning_rate),
        metrics=['accuracy'])

    return model




tuner = kt.Hyperband(model_builder,
                     objective='val_accuracy',
                     max_epochs=5,
                     factor=3
                     )
                     #directory='my_dir',
                     #project_name='intro_to_kt')


tuner.search(train_inputs, train_labels, epochs=2, validation_split=0.2, )


# Get the optimal hyperparameters
best_hps=tuner.get_best_hyperparameters(num_trials=1)[0]


model = tuner.hypermodel.build(best_hps)


model.fit(train_inputs, train_labels, epochs=10, validation_split=0.2)

model.save("LANDS")

'''