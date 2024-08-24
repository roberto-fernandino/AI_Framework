import csv
from time import sleep
import pandas as pd
from datetime import datetime

def is_normalized(value):
    """Verifica se o valor é um float entre 0 e 1, indicando que já está normalizado."""
    try:
        float_value = float(value)
        return 0.0 <= float_value <= 1.0
    except ValueError:
        return False

def fix_time(input_file):
    '''
        #Normaliza os timestamps de um arquivo CSV.
            :param input_file: Caminho para o arquivo CSV de entrada

    '''
    timestamps = []
    
    # Lê o CSV e coleta os timestamps
    with open(input_file, mode="r", newline="") as infile:
        reader = csv.reader(infile)
        header = next(reader)  # Pula o cabeçalho
        for row in reader:
            # Verifica se o valor já foi normalizado
            if is_normalized(row[0]):
                print(f"Valor já normalizado encontrado na linha: {row[0]}")
                continue  # Ignora linhas que já estão normalizadas
            date_time = datetime.strptime(row[0], "%Y-%m-%d")
            timestamps.append(date_time.timestamp())

    if not timestamps:
        print("Todos os valores já estão normalizados ou não há datas válidas.")
        return

    # Normaliza os timestamps
    min_timestamp = min(timestamps)
    max_timestamp = max(timestamps)
    normalized_timestamps = [
        (timestamp - min_timestamp) / (max_timestamp - min_timestamp)
        for timestamp in timestamps
    ]

    # Reescreve o CSV com os timestamps normalizados
    with open(input_file, mode="r", newline="") as infile:
        reader = csv.reader(infile)
        rows = list(reader)

    with open(input_file, mode="w", newline="") as outfile:
        writer = csv.writer(outfile)
        writer.writerow(rows[0])  # Escreve o cabeçalho novamente
        valid_row_index = 0  # Índice para acessar timestamps normalizados
        for i, row in enumerate(rows[1:], start=1):
            # Se o valor ainda não foi normalizado, substitua pela data normalizada
            if not is_normalized(row[0]):
                row[0] = normalized_timestamps[valid_row_index]
                valid_row_index += 1
            writer.writerow(row)


def add_moving_average(input_file, output_file, window_size, date_format=None):

    '''
        #Calcula a média móvel do preço de fechamento de um ativo.
            :param input_file: Caminho para o arquivo CSV de entrada
            :param output_file: Caminho para o arquivo CSV de saída
            :param window_size: Tamanho da janela para a média móvel
    '''
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


