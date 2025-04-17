import pdb
def calculate_discounted_price(price, discount):
    pdb.set_trace()
    if discount < 0 or discount > 100:
        raise ValueError("Discount must be between 0 and 100")
    return price * (1 - discount / 100)

try:
    price = float(input("Enter the orginial price: "))
    discount = float(input("Enter the discount percentage: "))
    final_price = calculate_discounted_price(price, discount)
    print("Final price is: ", final_price)
except ValueError as e:
    print(f"Error: {e}")

    