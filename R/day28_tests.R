library(testthat)

source("day28_functions.R")

test_that("Square root is correct", {
  expect_equal(safe_sqrt(9), 3)
})

test_that("Errors on invalid input", {
  expect_error(safe_sqrt(-1), "Cannot take square root")
  expect_error(safe_sqrt("banana"), "numeric")
})


test_that("Division works", {
  expect_equal(safe_divide(10, 2), 5)
})

test_that("Division error cases:", {
  expect_error(safe_divide(10, 0), "You cannot divide by zero. No one can.")
  expect_error(safe_divide("a", 5), "Both inputs must be numeric.")
  expect_error(safe_divide(5, "b"), "Both inputs must be numeric.")
})


test_that("average_scores works correctly.", {
  expect_equal(average_scores(c(90, 80, 70)), 80)
})

test_that("average_scores handles bad data", {
  expect_error(average_scores(c(101, 50)), "between 0 and 100")
  expect_error(average_scores(c()), "No scores provided.")
  expect_error(average_scores(c("banana", 90)), "numeric")
})


test_that("validate_score returns TRUE for valid input.", {
  expect_true(validate_score(88))
})

test_that("validate_score errors for non-numeric input.", {
  expect_error(validate_score("kiwi"))
})

test_that("validate_score errors for out of bounds input.", {
  expect_error(validate_score(199))
})

