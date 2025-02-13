from enum import auto, Enum

class TokenType(Enum):
    PLUS = auto()
    MINUS = auto()
    ASTERISK = auto()
    FSLASH = auto()
    LBRACKET = auto()
    RBRACKET = auto()
    NUM = auto()
    DOT = auto()

    def __init__(self, symbol):
        self.symbol = symbol

class Value:
    def __init__(self, vtype, value):
        self.__type = vtype
        self.__value = value

    def __repr__(self):
        return f"Value({self.__type}, {self.__value})"

    def get_type(self):
        return self.__type

    def get_value(self):
        return self.__value

class Token:
    def __init__(self, ttype, value=None, length=1):
        self.__ttype = ttype
        self.__value = value
        self.__length = length

    def __repr__(self):
        return f"Token({self.__ttype}, {self.__value})"

    def get_ttype(self):
        return self.__ttype

    def get_value(self):
        return self.__value

    def get_length(self):
        return self.__length

class Expr:
    def __init__(self, left, op, right):
        self.__left = left
        self.__right = right
        self.__op = op

    def __repr__(self):
        return f"Expr({self.__left}, {self.__right}, {self.__op})"

    def get_left(self):
        return self.__left

    def get_right(self):
        return self.__right

    def get_op(self):
        return self.__op

    def evaluate(self):
        left_value = self.__left.evaluate() if isinstance(self.__left, Expr) else float(self.__left.get_value())
        right_value = self.__right.evaluate() if isinstance(self.__right, Expr) else float(self.__right.get_value())
        match self.__op.get_ttype():
            case TokenType.PLUS:
                return left_value + right_value
            case TokenType.MINUS:
                return left_value - right_value
            case TokenType.ASTERISK:
                return left_value * right_value
            case TokenType.FSLASH:
                return left_value / right_value
            case _:
                raise ValueError("Unknown operator")

def parse_expression(tokens, curr):
    left, curr = parse_term(tokens, curr)
    
    while curr < len(tokens) and tokens[curr].get_ttype() in (TokenType.PLUS, TokenType.MINUS):
        op = tokens[curr]
        curr += 1
        right, curr = parse_term(tokens, curr)
        left = Expr(left, op, right)
    
    return left, curr

def parse_term(tokens, curr):
    left, curr = parse_factor(tokens, curr)
    
    while curr < len(tokens) and tokens[curr].get_ttype() in (TokenType.ASTERISK, TokenType.FSLASH):
        op = tokens[curr]
        curr += 1
        right, curr = parse_factor(tokens, curr)
        left = Expr(left, op, right)
    
    return left, curr

def parse_factor(tokens, curr):
    token = tokens[curr]
    
    if token.get_ttype() == TokenType.NUM:
        return token, curr + 1
    elif token.get_ttype() == TokenType.LBRACKET:
        curr += 1
        expr, curr = parse_expression(tokens, curr)
        curr = consume_token(tokens, curr, TokenType.RBRACKET, "Expected closing bracket")
        return expr, curr
    else:
        raise ValueError("Unexpected token")

def consume_token(tokens, curr, ttype, msg):
    if curr < len(tokens) and tokens[curr].get_ttype() == ttype:
        return curr + 1
    else:
        raise ValueError(msg)

def scan_num(source, curr):
    """Scans the source code for a number and returns the index of the last digit"""
    while curr < len(source) - 1:
        if source[curr + 1] in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']:
            curr += 1
        else:
            break

    if curr < len(source) - 1:
        if source[curr + 1] == '.':
            curr += 1

            while curr < len(source) - 1:
                if source[curr + 1] in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']:
                    curr += 1
                elif source[curr + 1] == '.':
                    raise ValueError("Number has two decimal points")
                else:
                    return curr
    
    return curr

def main():
    # User inputs the expression
    source = input(">>>")

    # Initialise the tokens list for tokens to be added to
    tokens = []

    # Intiialise the current index and the end index pointers
    curr = 0
    end = len(source)

    # Scanning
    while curr < end:
        match source[curr]:
            case '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9':
                initial = curr
                curr = scan_num(source, curr)
                tokens.append(Token(TokenType.NUM, source[initial:curr + 1], curr - initial + 1))
            case '+':
                tokens.append(Token(TokenType.PLUS))
            case '-':
                tokens.append(Token(TokenType.MINUS))
            case '*':
                tokens.append(Token(TokenType.ASTERISK))
            case '/':
                tokens.append(Token(TokenType.FSLASH))
            case '(':
                tokens.append(Token(TokenType.LBRACKET))
            case ')':
                tokens.append(Token(TokenType.RBRACKET))
            case '.':
                tokens.append(Token(TokenType.DOT))
            case ' ':
                pass
            case _:
                raise ValueError
        curr += 1

    # Parsing
    if tokens[0].get_ttype() == TokenType.LBRACKET:
        if tokens[-1].get_ttype() == TokenType.RBRACKET:
            tokens = tokens[1:-1]
        else:
            raise ValueError("Expected closing bracket")
    ast, _ = parse_expression(tokens, 0)
    print(ast)

    # Evaluation
    result = ast.evaluate()
    print(f"Result: {result}")

if __name__ == '__main__':
    main()
