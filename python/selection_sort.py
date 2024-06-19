import unittest

def find_smallest(array):
    smallest = array[0]
    smallest_index = 0

    for index in range(1, len(array)):
        if smallest > array[index]:
            smallest = array[index]
            smallest_index = index

    return smallest_index

def selection_sort(array):
    new_array = []

    for i in range(0, len(array)):
        smallest_index = find_smallest(array)

        new_array.append(array.pop(smallest_index))

    return new_array

class TestFindSmallest(unittest.TestCase):
    def test_array_with_one_item(self):
        array = [1]

        self.assertEqual(find_smallest(array), 0)

    def test_array_with_two_items_init(self):
        array = [1, 2]
        self.assertEqual(find_smallest(array), 0)
        
        array = [2, 1]
        self.assertEqual(find_smallest(array), 1)
 
    def test_array_with_three_items(self):
        array = [1, 2, 3]
        self.assertEqual(find_smallest(array), 0)

        array = [2, 1, 3]
        self.assertEqual(find_smallest(array), 1)

        array = [3, 2, 1]
        self.assertEqual(find_smallest(array), 2)

class TestSelectionSort(unittest.TestCase):
    def test_array_with_two_items(self):
        array = [2, 1]

        self.assertEqual(selection_sort(array), [1, 2])

    def test_array_with_three_items(self):
        array = [1, 2, 3]
        self.assertEqual(selection_sort(array), [1, 2, 3])

        array = [2, 1, 3]
        self.assertEqual(selection_sort(array), [1, 2, 3])

        array = [3, 2, 1]
        self.assertEqual(selection_sort(array), [1, 2, 3])

        array = [1, 3, 2]
        self.assertEqual(selection_sort(array), [1, 2, 3])

if __name__ == "__main__":
    unittest.main()
