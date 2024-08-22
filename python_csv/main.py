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
                continue
            date_time = datetime.strptime(row[0], "%Y-%m-%d")
            timestamps.append(date_time.timestamp())

    min_timestamp = min(timestamps)
    max_timestamp = max(timestamps)
    normalized_timestamps = [
        (timestamp - min_timestamp) / (max_timestamp - min_timestamp)
        for timestamp in timestamps
    ]
    with open(f"{input_file}.csv", mode="r", newline="") as infile:
        reader = csv.reader(infile)

        with open(f"{input_file}.csv", mode="w", newline="") as outfile:
            writer = csv.writer(outfile)
            for i, row in enumerate(reader):
                if i == 0:
                    writer.writerow(row)
                else:
                    row[0] = normalized_timestamps[i - 1]
                    writer.writerow(row)


def add_moving_average(input_file, output_file=None, window_size=5, date_format=None):
    # Read the CSV file
    df = pd.read_csv(input_file, index_col=False)

    # Convert the 'date' column to datetime
    if date_format:
        # If a specific date format is provided, use it
        df["date"] = pd.to_datetime(df["date"], format=date_format)
    else:
        # Try parsing the date without a specific format
        try:
            df["date"] = pd.to_datetime(df["date"])
        except ValueError:
            # If that fails, try some common formats
            common_formats = [
                "%Y-%m-%d",
                "%d-%m-%Y",
                "%m-%d-%Y",
                "%Y/%m/%d",
                "%d/%m/%Y",
                "%m/%d/%Y",
            ]
            for fmt in common_formats:
                try:
                    df["date"] = pd.to_datetime(df["date"], format=fmt)
                    print(f"Date parsed using format: {fmt}")
                    break
                except ValueError:
                    continue
            else:
                raise ValueError(
                    "Unable to parse the date column. Please specify the date_format."
                )

    # Set 'date' as the index
    df.set_index("date", inplace=True)

    # Convert price columns to float
    price_columns = ["open", "high", "low", "close"]
    for col in price_columns:
        df[col] = df[col].astype(float)

    # Calculate the moving average of the closing price
    df[f"MA_{window_size}"] = df["close"].rolling(window=window_size).mean()

    # Reset the index to make 'date' a column again
    df.reset_index(inplace=True)

    # Save to CSV if output_file is specified
    if output_file:
        df.to_csv(output_file, index=False)
        print(f"Data saved to {output_file}")

    return df


def main():
    # ma = int(input("Enter the window size for the moving average: "))
    # input_file = input("Enter the input file name: ")
    output_file = input("Enter the output file name: ")
    # add_moving_average(f"{input_file}.csv", f"{output_file}.csv", window_size=ma)
    sleep(1)
    fix_time(output_file)


if __name__ == "__main__":
    main()
