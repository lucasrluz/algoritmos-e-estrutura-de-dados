import unittest

def quicksort(array):
    if len(array) < 2:
        return array

    pivot = array[0]
    less = [i for i in array[1:] if i <= pivot]
    greater = [i for i in array[1:] if i > pivot]

    return quicksort(less) + [pivot] + quicksort(greater)

class TestQuicksort(unittest.TestCase):
    def test_should_return_1_2_3_4_and_5(self):
        array = [4, 1, 3, 2, 5]
        self.assertEqual(quicksort(array), [1, 2, 3, 4, 5])

if __name__ == "__main__":
    unittest.main()

