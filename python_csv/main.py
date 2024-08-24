import os
from time import sleep
import pandas as pd
from Division_and_Process import dividir_csv, create_target_file
from FixTime_and_MovingAverage import fix_time, add_moving_average

def main():

    '''

    # Os arquivos CSV devem estar na pasta data
    # Os arquivos de saída serão salvos nas pastas targets e data_ia
    # Serão criados 7 arquivos de saída:
    # 1 arquivo - moving_average_{arquivo_csv}.csv -- Pasta data_ia
    # 3 arquivos - {arquivo_csv}_treino_70.csv, {arquivo_csv}_teste_15.csv, {arquivo_csv}_validacao_15.csv -- Pasta data_ia
    # 3 arquivos- target_{arquivo_csv}.csv -- Pasta targets

    '''
    escolha_trade = int(input("[0] - CSV Swing Trade\n[1] - CSV Day Trade\n[2] - Sair\n"))
    while(escolha_trade != 2):
        if escolha_trade == 0:
            arquivo_csv = str(input("Digite o nome do arquivo CSV para SwingTrade -- Sem a extensão .csv: "))

            # Diretório onde os arquivos CSV estão localizados
            dir_data = "../data/"
            caminho_completo_csv = os.path.join(dir_data, f"{arquivo_csv}.csv")

            escolha_preparacao_arquivo = int(input("[0] - Voltar\n[1] - Fix Time\n[2] - Add Moving Average\n[3] - Create Target File\n[4] - Division of the .csv file\n[5] - Full Preparation\n"))
            
            # Definindo os diretórios de saída
            dir_targets = "../targets"
            dir_data_ia = "../data_ia"

            # Certificando-se de que os diretórios existem
            os.makedirs(dir_targets, exist_ok=True)
            os.makedirs(dir_data_ia, exist_ok=True)

            if escolha_preparacao_arquivo == 0:
                continue
            elif escolha_preparacao_arquivo == 1:
                fix_time(caminho_completo_csv)
            elif escolha_preparacao_arquivo == 2:
                ma = int(input("Enter the window size for the moving average: "))
                add_moving_average(caminho_completo_csv, os.path.join(dir_data_ia, f"moving_average_{arquivo_csv}.csv"), window_size=ma)
            elif escolha_preparacao_arquivo == 3:
                print("Remember to use the same window size that was used to the moving average!")
                ma = int(input("Enter the window size for the moving average: "))
                create_target_file(caminho_completo_csv, window_size=ma)
            elif escolha_preparacao_arquivo == 4:
                dividir_csv(caminho_completo_csv,
                            os.path.join(dir_data_ia, f"{arquivo_csv}_treino_70.csv"),
                            os.path.join(dir_data_ia, f"{arquivo_csv}_teste_15.csv"),
                            os.path.join(dir_data_ia, f"{arquivo_csv}_validacao_15.csv"))
            elif escolha_preparacao_arquivo == 5:
                ma = int(input("Enter the window size for the moving average: "))
                add_moving_average(caminho_completo_csv, os.path.join(dir_data_ia, f"moving_average_{arquivo_csv}.csv"), window_size=ma)
                sleep(1)
                fix_time(caminho_completo_csv)
                sleep(1)
                dividir_csv(caminho_completo_csv,
                            os.path.join(dir_data_ia, f"{arquivo_csv}_treino_70.csv"),
                            os.path.join(dir_data_ia, f"{arquivo_csv}_teste_15.csv"),
                            os.path.join(dir_data_ia, f"{arquivo_csv}_validacao_15.csv"))
                nome_arquivos = [
                    os.path.join(dir_data_ia, f"{arquivo_csv}_treino_70.csv"),
                    os.path.join(dir_data_ia, f"{arquivo_csv}_teste_15.csv"),
                    os.path.join(dir_data_ia, f"{arquivo_csv}_validacao_15.csv")
                ]
                print(nome_arquivos)
                sleep(1)
                for arquivo in nome_arquivos:
                    create_target_file(arquivo, window_size=ma)

        elif escolha_trade == 1:
            #arquivo_csv = str(input("Digite o nome do arquivo CSV para DayTrade: "))
            print("Coming Soon..")
            escolha_trade = int(input("[0] - CSV Swing Trade\n[1] - CSV Day Trade\n[2] - Sair\n"))
            
        elif escolha_trade == 2:
            print("Exiting..")
            return
        else:
            print("Opção inválida - Tente Novamente!")
            escolha_trade = int(input("[0] - CSV Swing Trade\n[1] - CSV Day Trade\n[2] - Sair\n"))
main()
