import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers


import json
import os
import numpy as np




def get_decks_and_positions():

    fobj = open("decksandpositions.json", "rb")
    data = json.load(fobj)


    length = len(data)

    #[main:  [deck: [cards:], [pos: ] ]  

    ohvlen = len( data[0][0] )


    decks = np.zeros((length, ohvlen))
    positions = np.zeros((length, 2))


    deckid = 0

    for deck in data:
        #print(deck)

        decks[deckid] = deck[0]

        positions[deckid] = deck[1]

        """
        for card in deck[0]:
            decks[deckid] = card


        for pos in deck[1]:

            positions[deckid] = pos
        """
        
        deckid +=1

    

    return ( decks, positions )




(train_inputs, train_labels) = get_decks_and_positions()


"""print(train_inputs.shape)
print(train_labels.shape)




inputs = keras.Input(shape=(1899))
m1 = layers.Dense(1000, activation="relu") (inputs)


o = layers.Dense(1000, activation="relu") (m1)
outputs = layers.Dense( 2 )(o)


model = keras.Model(inputs=inputs, outputs=outputs, name="majormodel")
"""




model = keras.models.load_model('MODEL')






model.compile(
    loss= keras.losses.MeanSquaredError(),
    optimizer=tf.keras.optimizers.Adam(),
    metrics=["accuracy"],
)



model.fit(train_inputs, train_labels, batch_size=100, epochs=3)



model.save('MODEL')

'''




GCS_PATH_FOR_SAVED_MODEL = 'gs://vertex-ai-test-5/somename'
localmodel = 'TEMPMODEL'

model.save('MODEL')


'''