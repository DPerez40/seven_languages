safe_sqrt <- function(x) {
  if (!is.numeric(x)) {
    stop("Input must be numeric.")
  }
  if (x < 0) {
    stop("Cannot take square root of a negative number.")
  }
  sqrt(x)
}

safe_divide <- function(x, y) {
  if(!is.numeric(x) || !is.numeric(y)) {
    stop("Both inputs must be numeric.")
  }
  
  if (y ==0) {
    stop("You cannot divide by zero. No one can.")
  }
  
  return(x / y)
}


average_scores <- function(scores){
  if (!all(sapply(scores, is.numeric))) {
    stop("All inputs must be numeric.")
  }
  
  if (length(scores) == 0) {
    stop("No scores provided.")
  }
  
  if (any(scores < 0 | scores > 100)) {
    stop("Invalid score: must be between 0 and 100.")
  }
  
  mean(scores)
}


validate_score <- function(score) {
  if (!is.numeric(score)) {
    stop("Input must be numeric.")
  }
  
  if (score < 0 || score > 100) {
    stop("Number must be between 0 and 100.")
  }
  
  return(TRUE)
}



