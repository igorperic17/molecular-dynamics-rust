import numpy
import csv

INPUT_FILE_PATH = "data_md17/npz_data/rmd17_aspirin.npz"
OUTPUT_FILE_PATH = "data_md17/npz_data/rmd17_aspirin.csv"

HEADER_FIELDS = ["p_x", "p_y", "p_z", "charge", "energy"]

with numpy.load(INPUT_FILE_PATH) as data:
    lst = data.files

    with open(OUTPUT_FILE_PATH, "w") as csv_file:
        csv_writer = csv.writer(csv_file)
        csv_writer.writerow(HEADER_FIELDS)
        
        coords = data['coords']
        charges = data['nuclear_charges']
        energies = data['energies']
        
        print(coords.shape)
        print(charges.shape)
        print(energies.shape)
        
        # row = [coords
        
        # for item in lst:
        #     print(item)
        #     print(data[item])
            
        
    
        
        
