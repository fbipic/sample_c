/***************************************
 *
 * Copyright 2025 Fabio Piccolo
 *
 */

#include "CppUTest/TestHarness.h"

TEST_GROUP(HelloTestGroup)
{
  void setup(){}
  void teardown(){}
};

TEST(HelloTestGroup, HelloTest)
{
  int res = 1;
//   FAIL("Fail me!");
  CHECK_EQUAL(0, res);
}
