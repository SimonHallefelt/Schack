import sklearn
import pickle


hidden_layers = 2
neurons = 128
epochs = 100
batch_size = 32


file = open("data\\data.txt", "r")
data = file.readlines()
file.close()


