import pandas as pd
import os

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
    


def create_target_file(arquivo_original, window_size):

    '''
        Cria um arquivo CSV com as categorizações de variação percentual para diferentes horizontes temporais.
            :param arquivo_original: Caminho para o arquivo CSV de entrada
            :param window_size: Tamanho da janela para a média móvel
    '''
    # Carrega o arquivo CSV
    df = pd.read_csv(arquivo_original)

    # Verifica se a coluna 'MA_{window_size}' existe no DataFrame
    ma_column = f'MA_{window_size}'
    # Preenche valores ausentes com NaN e remove as linhas que possuem NaN
    df[ma_column] = pd.to_numeric(df[ma_column], errors='coerce')
    df.dropna(subset=[ma_column], inplace=True)
    
    
    df = df[:-1]

    # Calcula a variação percentual para diferentes horizontes temporais
    df[f'{ma_column}_change_short'] = df[ma_column].pct_change(periods=3) * 100  # Curto prazo (1-3 dias)
    df[f'{ma_column}_change_medium'] = df[ma_column].pct_change(periods=10) * 100  # Médio prazo (1-2 semanas)
    df[f'{ma_column}_change_long'] = df[ma_column].pct_change(periods=60) * 100  # Longo prazo (1-3 meses)

    # Aplica a categorização para cada horizonte temporal
    df['target_short'] = df[f'{ma_column}_change_short'].apply(categorize_change)
    df['target_medium'] = df[f'{ma_column}_change_medium'].apply(categorize_change)
    df['target_long'] = df[f'{ma_column}_change_long'].apply(categorize_change)

    # Cria um novo DataFrame com as colunas de interesse
    df_target = df[['date', 'target_short', 'target_medium', 'target_long']]

    # Verifica e cria o diretório de saída se necessário
    dir_targets = "../targets"
    os.makedirs(dir_targets, exist_ok=True)
    
    # Define o caminho completo do arquivo de saída
    output_file = os.path.join(dir_targets, f'target_{os.path.basename(arquivo_original)}')

    # Salva o resultado em um novo arquivo CSV
    df_target.to_csv(output_file, index=False)
    print(f"Arquivo salvo com sucesso em {output_file}")



