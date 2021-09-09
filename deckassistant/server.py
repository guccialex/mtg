import tensorflow as tf
from tensorflow import keras
from tensorflow.keras import layers

import json
import os
import numpy as np

import mtgmodule


predictionmodel = keras.models.load_model('PREDICTION')


#landsmodel = keras.models.load_model('LANDS')

import autokeras as ak

landsmodel = keras.models.load_model("LANDS", custom_objects=ak.CUSTOM_OBJECTS)


#given a list of cards
#return a list of cards
def cardstoprediction(cardnames):

    oh = mtgmodule.cardstoonehot(cardnames)
    oh = np.array([oh])
    onehotprediction = predictionmodel.predict( oh )

    #return the order of cards
    return mtgmodule.onehottocardorder(onehotprediction[0])



def cardstolands(cardnames):

    oh = mtgmodule.cardstoonehot(cardnames)
    oh = np.array([oh])
    onehotprediction = landsmodel.predict( oh )
    return mtgmodule.onehottocards(onehotprediction[0])







from flask import Flask, request, jsonify, Response
from flask_cors import CORS

app = Flask(__name__)
CORS(app)


@app.route('/mtgapi/land_prediction', methods=['POST'])
def land_prediction():
    
    content = request.json
    print(content)

    result = cardstolands(content)
    response = jsonify(result)

    header = response.headers
    header['Access-Control-Allow-Origin'] = '*'

    return response



@app.route('/mtgapi/card_prediction', methods=['POST'])
def card_prediction():
    
    content = request.json

    result = cardstoprediction(content)
    response = jsonify(result)

    header = response.headers
    header['Access-Control-Allow-Origin'] = '*'

    return response




@app.route('/health', methods=['GET'])
def health():

    return "OK"


if __name__ == '__main__':
    app.run(host= '0.0.0.0')

