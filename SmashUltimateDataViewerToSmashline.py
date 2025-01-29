#Name: Gewl
#Date: March 12, 2023
#Title: Smash Ultimate Data Viewer Code to Smashline converter
#Description: Allows you to copy and paste code from Smash Ultimate Data Viewer and have it be converted to Smashline

code_to_convert = input()

def code_conversion(data_viewer_code):

    numerical_values = [0,1,3,4,5,6,7,8,9,10,11,15,16,20,21,22]
    hash40_group = [2,32]
    none_group = [12,13,14]
    equals_to_comma_group = [17,18,19,23,24,25,26,27,28,29,30,31,33,34,35]

    data_viewer_code = data_viewer_code.split()

    for i in range(len(data_viewer_code)):
        val = 0
        current_pos = data_viewer_code[i]
        if i in numerical_values:
            for c in current_pos:
                 if c.isdigit():
                    val = val * 10 + int(c)
                    data_viewer_code[i] = data_viewer_code[i].replace(data_viewer_code[i],str(val))
                    if "." in current_pos:
                        data_viewer_code[i] = data_viewer_code[i].replace(data_viewer_code[i],str(val/10))
        if i in hash40_group:
            BetweenBrackets = data_viewer_code[i][data_viewer_code[i].find("(")+1:data_viewer_code[i].find(")")]
            data_viewer_code[i] = "Hash40::new(" + BetweenBrackets + ")"
        if i in none_group:
            data_viewer_code[i] = "None"
        if i in equals_to_comma_group:
            BetweenEqualsAndComma = data_viewer_code[i][data_viewer_code[i].find("=")+1:data_viewer_code[i].find(",")]
            if BetweenEqualsAndComma.isupper():
             data_viewer_code[i] = "*" + BetweenEqualsAndComma
            else:
             data_viewer_code[i] = BetweenEqualsAndComma
    
    data_viewer_code = str(data_viewer_code)
    data_viewer_code = data_viewer_code.replace("'","")
    data_viewer_code = data_viewer_code.replace("[","")
    data_viewer_code = data_viewer_code.replace("]","")
    data_viewer_code = "macros::ATTACK(agent," + data_viewer_code + ");"
    



    return data_viewer_code


#code_conversion(code_to_convert)
print("\n"+code_conversion(code_to_convert))