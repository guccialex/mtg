import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers


import json
import os
import numpy as np



def get_ml_decks():

    fobj = open("mldecks.json", "rb")
    data = json.load(fobj)



    length = len(data)

    considereddecks =  10
    length = considereddecks

    numberofmutations = len(data[0][0]) #26


    train_inputs = np.zeros((length*(numberofmutations-1), 1899))
    train_labels = np.zeros((length*(numberofmutations-1), 1899))

    test_inputs  = np.zeros((length, 1899))
    test_labels  = np.zeros((length, 1899))


    #print(train_inputs.shape)
    #print(train_labels.shape)

    #print(test_inputs.shape)
    #print(test_labels.shape)


    curtrain = 0
    curtest = 0



    for mutationsanddeck in data:

        length = length -1

        if length < 0:
            break


        firstmutation = True
        
        for mutation in mutationsanddeck[0]:

            if firstmutation == False:
            
                for cardid in mutation:
                    train_inputs[curtrain][cardid] = 1.0
                for cardid in mutationsanddeck[1]:
                    train_labels[curtrain][cardid] = 1.0
                
                curtrain += 1
                #print("curtrain")
                #print(curtrain)

            else:

                for cardid in mutation:
                    test_inputs[curtest][cardid] = 1.0
                for cardid in mutationsanddeck[1]:
                    test_labels[curtest][cardid] = 1.0

                curtest += 1
                #print(curtest)


            firstmutation = False

    return ( train_inputs, train_labels, test_inputs, test_labels)



def get_ml_decks_unmutated():


    fobj = open("mldecks_no_mutations.json", "rb")
    data = json.load(fobj)


    length = len(data)

    length = 10
    testlength = 0


    train_inputs = np.zeros((length-testlength, 1899))
    train_labels = np.zeros((length-testlength, 1899))

    test_inputs  = np.zeros((testlength, 1899))
    test_labels  = np.zeros((testlength, 1899))


    #print(train_inputs.shape)
    #print(train_labels.shape)
    #print(test_inputs.shape)
    #print(test_labels.shape)


    testleft = testlength


    for deckid in range(length):


        if testleft != 0:

            for card in data[deckid][0]:
                cardid = card[0]
                cardscore = card[1]

                #print(data[deckid][0])
                #print(cardid)

                test_inputs[deckid][cardid] = cardscore
                test_labels[deckid][cardid] = cardscore

            testleft = testleft -1

        else:

            for card in data[deckid][0]:

                cardid = card[0]
                cardscore = card[1]

                train_inputs[deckid-testlength][cardid] = cardscore
                train_labels[deckid-testlength][cardid] = cardscore

    return ( train_inputs, train_labels, test_inputs, test_labels)







(train_inputs, train_labels, test_inputs, test_labels ) = get_ml_decks_unmutated()










inputs = keras.Input(shape=(1899))
m1 = layers.Dense(1000, activation="relu") (inputs)

'''
m1 = layers.Reshape( (20,20,1) )(m1)
m1 = layers.Conv2D(  2, (4,4) )(m1)
m1 = layers.Conv2D(  2, (4,4) )(m1)
m1 = layers.Conv2D(  2, (4,4) )(m1)
m1 = layers.Conv2D(  2, (4,4) )(m1)
m1 = layers.Flatten( )(m1)
'''

o = layers.Dense(1000, activation="relu") (m1)
outputs = layers.Dense( 1899 )(o)


model = keras.Model(inputs=inputs, outputs=outputs, name="mtgmodel")



x = tf.constant(1, shape=(1,1899))
    
y = np.zeros(1899)
y[0] = 1899
print(tf.constant(y))


# Define custom loss
def custom_loss(y_true, y_pred):


    return keras.losses.MeanSquaredError().call(y_true, y_pred) 



"""
print("----")


a = tf.random.normal(shape=(2, 2))
b = tf.random.normal(shape=(2, 2))

a = tf.convert_to_tensor([1,1],  dtype=tf.float32)
b = tf.convert_to_tensor([0,0],  dtype=tf.float32)

with tf.GradientTape() as tape:
    tape.watch(a)  # Start recording the history of operations applied to `a`
    c = tf.sqrt(tf.square(a) + tf.square(b))  # Do some math using `a`
    # What's the gradient of `c` with respect to `a`?
    dc_da = tape.gradient(c, a)
    print(dc_da)


print("----")
"""

'''
toreturn = tf.constant(0.1, shape=(1,) )
print(toreturn)
tf.print(toreturn)
'''

model.compile(
    loss= custom_loss, #keras.losses.MeanSquaredError(),
    optimizer=keras.optimizers.Adam(),
    metrics=["accuracy"],
)



model.fit(train_inputs, train_labels, batch_size=1, epochs=1)

'''
print("Evaluate on test data")
results = model.evaluate(test_inputs, test_labels, batch_size=1)
print("")
print("test loss, test acc:", results)
print("")
'''

GCS_PATH_FOR_SAVED_MODEL = 'gs://vertex-ai-test-5/somename'
localmodel = 'TEMPMODEL'

model.save('MODEL')

