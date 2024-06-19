import unittest

def binary_search(array, item):
    low = 0
    high = len(array) - 1

    while low <= high:
        mid = (low + high) // 2

        if item == array[mid]:
            return mid

        if item < array[mid]:
            high = mid - 1

        if item > array[mid]:
            low = mid + 1

    return None

class TestBinarySearch(unittest.TestCase):
    def test_array_with_one_item(self):
        array = [1]

        self.assertEqual(binary_search(array, 1), 0)
        self.assertEqual(binary_search(array, -3), None)
        self.assertEqual(binary_search(array, 3), None)

    def test_array_with_two_item(self):
        array = [1, 3]

        self.assertEqual(binary_search(array, 1), 0)
        self.assertEqual(binary_search(array, 3), 1)
        self.assertEqual(binary_search(array, -5), None)
        self.assertEqual(binary_search(array, 5), None)

    def test_array_with_three(self):
        array = [1, 3, 5]

        self.assertEqual(binary_search(array, 1), 0)
        self.assertEqual(binary_search(array, 3), 1)
        self.assertEqual(binary_search(array, 5), 2)
        self.assertEqual(binary_search(array, -7), None)
        self.assertEqual(binary_search(array, 7), None)

if __name__ == '__main__':
        unittest.main()


