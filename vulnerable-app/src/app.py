import pickle
import sqlite3
from flask import Flask, request

app = Flask(__name__)
DEBUG = True
SECRET_KEY = "super-secret-key-12345"

# AI bare except — swallows all errors
@app.route("/api/data")
def get_data():
    try:
        data = fetch_from_db()
        return data
    except:
        print("Something went wrong")

# AI SQL injection via f-string
@app.route("/api/users")
def get_user():
    user_id = request.args.get("id")
    conn = sqlite3.connect("db.sqlite")
    cursor = conn.cursor()
    cursor.execute(f"SELECT * FROM users WHERE id = {user_id}")
    return cursor.fetchone()

# AI unsafe pickle deserialization
@app.route("/api/load-model")
def load_model():
    model = pickle.load(open("model.pkl", "rb"))
    return str(model.predict([1, 2, 3]))

if __name__ == "__main__":
    app.run(debug=True)
