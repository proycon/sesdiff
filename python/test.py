#!/usr/bin/env python3

import unittest

from sesdiff import shortest_edit_script, Mode

class Tests(unittest.TestCase):
    def test_normal(self):
        result = shortest_edit_script("hablaron","hablar")
        print(result)
        self.assertEqual(result.distance(),2)
        self.assertEqual(len(result),2)
        self.assertEqual(result[0], ('=',"hablar"))
        self.assertEqual(result[1], ('-',"on"))
        for instruction in result:
            print(instruction)


    def test_suffix(self):
        result = shortest_edit_script("hablaron","hablar", Mode.SUFFIX)
        self.assertEqual(result.distance(),2)
        self.assertEqual(len(result),1)
        self.assertEqual(result[0], ('-',"on"))

    def test_normal_cyrillic(self):
        result = shortest_edit_script("говорим","говорить")
        self.assertEqual(result.distance(),3)
        self.assertEqual(len(result),3)
        self.assertEqual(result[0], ('=',"говори"))
        self.assertEqual(result[1], ('-',"м"))
        self.assertEqual(result[2], ('+',"ть"))

if __name__ == "__main__":
    unittest.main()
