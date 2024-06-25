import unittest

def fat(number):
    if number == 1:
        return number

    return number * fat(number - 1)

class TestFat(unittest.TestCase):
    def test_should_return_120(self):
        self.assertEqual(fat(5), 120)

    def test_should_return(self):
        self.assertEqual(fat(10), 3628800)

if __name__ == "__main__":
    unittest.main()
