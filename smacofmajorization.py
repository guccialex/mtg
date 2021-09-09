from sklearn.manifold import smacof
import matplotlib.pyplot as plt

print("doing smacof majorization")

import json
import os


#fobj = open("deckcolours.json", "rb")
#colours = json.load(fobj)


fobj = open("dissimilaritiesmatrix.json", "rb")
data = json.load(fobj)


result = smacof( data )#,metric=True )# , max_iter=20000, eps=1e-9 , n_init=3)


deckresults = []



deckid = 0

for x in result[0]:


    deckresults.append( [ x[0], x[1]]  )
    
    #print(color)
    #plt.plot(  x[0], x[1]  , marker='o', color=colours[deckid])
    

    deckid += 1


#plt.axis([-1, 1, -1, 1])
#plt.show()
#plt.show()

with open('smacofdeckpositions.json', 'w') as outfile:
    json.dump(deckresults, outfile)

