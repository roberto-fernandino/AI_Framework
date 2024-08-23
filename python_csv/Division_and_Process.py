import pandas as pd

def dividir_csv(arquivo_original, arquivo_70, arquivo_15_1, arquivo_15_2):

    df = pd.read_csv(arquivo_original)

    # Calcula os tamanhos das divisões
    n = len(df)
    n_70 = int(0.7 * n)
    n_15 = int(0.15 * n)

    # Divide o dataframe de acordo com os tamanhos calculados
    df_70 = df[:n_70]
    df_15_1 = df[n_70:n_70 + n_15]
    df_15_2 = df[n_70 + n_15:]

    # Salva os arquivos divididos
    df_70.to_csv(arquivo_70, index=False)
    df_15_1.to_csv(arquivo_15_1, index=False)
    df_15_2.to_csv(arquivo_15_2, index=False)

#dividir_csv('btc-swing-2.csv', 'arquivo_treino_70.csv', 'arquivo_teste_15.csv', 'arquivo_validacao_15.csv')


def categorize_change(change):
    if change >= 10:
        return 1.0
    elif 5 <= change < 10:
        return 0.5
    elif 2 <= change < 5:
        return 0.25
    elif -2 <= change < 2:
        return 0.0
    elif -5 <= change < -2:
        return -0.25
    elif -10 <= change < -5:
        return -0.5
    else:
        return -1.0

def process_csv(arquivo_csv):
    # Carrega o arquivo CSV
    df = pd.read_csv(arquivo_csv)

    # Preenche valores ausentes com NaN e remove as linhas que possuem NaN
    df['MA_2'] = pd.to_numeric(df['MA_2'], errors='coerce')
    df.dropna(subset=['MA_2'], inplace=True)

    df['MA_2_change_short'] = df['MA_2'].pct_change(periods=3) * 100  # Curto prazo (1-3 dias)
    df['MA_2_change_medium'] = df['MA_2'].pct_change(periods=10) * 100  # Médio prazo (1-2 semanas)
    df['MA_2_change_long'] = df['MA_2'].pct_change(periods=60) * 100  # Longo prazo (1-3 meses)

    # Aplica a categorização para cada horizonte temporal
    df['target_short'] = df['MA_2_change_short'].apply(categorize_change)
    df['target_medium'] = df['MA_2_change_medium'].apply(categorize_change)
    df['target_long'] = df['MA_2_change_long'].apply(categorize_change)

    df_target = df[['date', 'target_short', 'target_medium', 'target_long']]

    # Salva o resultado em um novo arquivo CSV
    df_target.to_csv('arquivo_target_horizontes.csv', index=False)

    #print(df_target.head(10))

# Exemplo de uso
arquivo_original = 'btc-swing-2.csv'
process_csv(arquivo_original)
