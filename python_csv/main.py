import csv
from time import sleep
import pandas as pd
from datetime import datetime


def fix_time(input_file):
    timestamps = []
    with open(f"{input_file}.csv", mode="r", newline="") as infile:
        reader = csv.reader(infile)
        for i, row in enumerate(reader):
            if i == 0:
                continue  # Skip header
            date_time = datetime.strptime(row[0], "%Y-%m-%d")
            timestamps.append(date_time.timestamp())

    min_timestamp = min(timestamps)
    max_timestamp = max(timestamps)
    normalized_timestamps = [
        (timestamp - min_timestamp) / (max_timestamp - min_timestamp)
        for timestamp in timestamps
    ]

    # Rewriting the CSV with normalized timestamps
    with open(f"{input_file}.csv", mode="r", newline="") as infile:
        reader = csv.reader(infile)
        rows = list(reader)  # Read all rows into a list

    with open(f"{input_file}.csv", mode="w", newline="") as outfile:
        writer = csv.writer(outfile)
        for i, row in enumerate(rows):
            if i == 0:
                writer.writerow(row)  # Write header
            else:
                row[0] = normalized_timestamps[i - 1]
                writer.writerow(row)




def add_moving_average(input_file, output_file=None, window_size=5, date_format=None):
    # Ler o arquivo CSV
    df = pd.read_csv(input_file)
    print("DataFrame antes de processar:", df.head())

    # Verificar e converter a coluna 'date' para datetime
    if 'date' in df.columns:
        df["date"] = pd.to_datetime(df["date"], format=date_format)
    else:
        print("Coluna 'date' não encontrada.")
        return

    # Definir 'date' como o índice
    df.set_index("date", inplace=True)

    # Converter colunas de preço para float
    price_columns = ["open", "high", "low", "close"]
    for col in price_columns:
        df[col] = df[col].astype(float)

    # Calcular a média móvel do preço de fechamento
    df[f"MA_{window_size}"] = df["close"].rolling(window=window_size).mean()

    # Resetar o índice para tornar 'date' uma coluna novamente
    df.reset_index(inplace=True)
    print("DataFrame após calcular a média móvel:", df.head())

    # Salvar para CSV se 'output_file' for especificado
    if output_file:
        df.to_csv(output_file, index=False)
        print(f"Dados salvos em {output_file}")

    return df

def main():
    ma = int(input("Enter the window size for the moving average: "))
    input_file = input("Enter the input file name: ")
    output_file = input("Enter the output file name: ")
    add_moving_average(f"{input_file}.csv", f"{output_file}.csv", window_size=ma)
    sleep(1)
    fix_time(output_file)


if __name__ == "__main__":
    main()
