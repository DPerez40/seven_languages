grade_score <- function(score) {
  tryCatch({
    if(!is.numeric(score)) {
      stop("Input must be numeric.")
    }
    
    if(score > 100) {
      stop("Impossible.")
    }
    
    if(score < 0) {
      stop("Impossible to be below 0.")
    }
    
    
    if (score >= 90) {
      cat("A\n")
    }
    
    else if(score >= 80) {
      cat("B\n")
    }
    
    else if(score >= 70) {
      cat("C\n")
    }
    
    else if(score >=60) {
      cat("D\n")
    }
    
    else {
      cat("F\n")
    }
    
    cat("Valid input", score, "\n")
    
  }, error = function(e) {
    cat("Validation Error", conditionMessage(e), "\n")
  }, finally = {
    cat("Validation complete.\n")
  }
  )
}

