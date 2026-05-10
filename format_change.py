def formatChange(cents):
    dollars = cents // 100
    cents = cents % 100

    quarters = cents // 25
    cents = cents % 25

    dimes = cents // 10
    cents = cents % 10

    nickels = cents // 5
    cents = cents % 5

    pennies = cents // 1
    cents = cents % 1
    
    parts = []
    
    if dollars > 0:
        parts.append(f"{dollars} {'dollar' if dollars == 1 else 'dollars'}")
    if quarters > 0:
        parts.append(f"{quarters} {'quarter' if quarters == 1 else 'quarters'}")
    if dimes > 0:
        parts.append(f"{dimes} {'dime' if dimes == 1 else 'dimes'}")
    if nickels > 0:
        parts.append(f"{nickels} {'nickel' if nickels == 1 else 'nickels'}")
    if pennies > 0:
        parts.append(f"{pennies} {'penny' if pennies == 1 else 'pennies'}")

    if len(parts) == 0:
        return "no change"
    elif len(parts) == 1:
        return parts[0]
    else:
        return ", ".join(parts[:-1]) + " and " + parts[-1]

#Basic Cases
print(formatChange(1))
print(formatChange(15))
print(formatChange(25))
print(formatChange(100))
print(formatChange(141))

#Singular vs Plural
print(formatChange(2))
print(formatChange(20))
print(formatChange(50))
print(formatChange(200))

#Multiple Units with Commas
print(formatChange(387))
print(formatChange(176))
print(formatChange(115))

#Skipping Zero Values
print(formatChange(101))
print(formatChange(300))
print(formatChange(30))

#Edge Cases
print(formatChange(0))
print(formatChange(100000))