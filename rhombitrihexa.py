import matplotlib.pyplot as plt
import numpy as np

sq3 = np.sqrt(3)

base = np.array([
    [-0.5, sq3 / 2],
    [0.5, sq3 / 2],
    [1, 0],
    [0.5, -sq3 / 2],
    [-0.5, -sq3 / 2],
    [-1, 0],
])

a1 = np.array([(3 + sq3) / 2, (1 + sq3) / 2])
a2 = np.array([0, sq3 + 1])

points = np.concatenate([base + i * a1 + j * a2 for i in range(100) for j in range(100)])

clusters = np.loadtxt("foo.txt", dtype=int)

for i in range(np.shape(clusters)[0]):
    plt.scatter(points[clusters[i,:], 0], points[clusters[i,:], 1])

plt.show()

# mask = np.linalg.norm(points - 50*(a1 + a2), axis = 1) < (sq3+1)*10
# print(np.shape(mask))
# filtered = points[mask]
# print(np.shape(filtered))
#
# plt.scatter(filtered[:,0], filtered[:,1])
# plt.show()
#
