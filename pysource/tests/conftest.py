import pytest
import csv
import json
import yaml


@pytest.fixture
def artists_csv_str():
    data = """
name,age,genre,albums_sold,hometown,active,label
John,34,Rock,10000,New York,True,Sony
Paul,36,Pop,,Liverpool,True,EMI
George,35,Rock,8000,Liverpool,False,EMI
Ringo,33,Pop,7000,Liverpool,True,EMI
Mick,37,Rock,12000,London,True,Decca
Keith,35,Rock,11000,London,True,Decca
Charlie,33,Jazz,6000,London,True,Decca
Ronnie,32,Blues,5000,London,True,Decca
Roger,34,Rock,9000,London,True,Track
Pete,35,Rock,8500,London,True,Track
John,33,Pop,7500,London,True,Track
Keith,32,Rock,6500,London,True,Track
Robert,34,Rock,9500,London,True,Atlantic
Jimmy,35,Blues,8000,London,True,Atlantic
John Paul,33,Rock,7000,London,True,Atlantic
"""
    return data.strip()


@pytest.fixture
def artists_csv():
    return csv.reader(artists_csv_str().splitlines())


@pytest.fixture
def cars_json_str():
    data = """
[
    {"make": "Toyota", "model": "Corolla", "year": 2005, "color": "Blue", "price": 5000},
    {"make": "Honda", "model": "Civic", "year": 2006, "color": "Red", "price": 6000},
    {"make": "Ford", "model": "Focus", "year": 2007, "color": "White", "price": 7000},
    {"make": "Chevrolet", "model": "Cruze", "year": 2008, "color": "Black", "price": 8000},
    {"make": "Hyundai", "model": "Elantra", "year": 2009, "color": "Silver", "price": 9000},
    {"make": "Nissan", "model": "Sentra", "year": 2010, "color": "Blue", "price": 10000},
    {"make": "Volkswagen", "model": "Jetta", "year": 2011, "color": "Red", "price": 11000},
    {"make": "Subaru", "model": "Impreza", "year": 2012, "color": "White", "price": 12000},
    {"make": "Mazda", "model": "3", "year": 2013, "color": "Black", "price": 13000},
    {"make": "Kia", "model": "Forte", "year": 2014, "color": "Silver", "price": 14000}
]
"""
    return data


@pytest.fixture
def cars_json():
    return json.loads(cars_json_str())


@pytest.fixture
def toys_yaml_str():
    data = """
- name: Teddy Bear 🧸
  type: Stuffed Animal
  color: Brown
  age_range: 1-3
  price: 20
  manufacturer: Hasbro
  in_stock: true
  weight: 0.5
  dimensions: "12 x 8 x 5"
  safety_certified: true
  country_of_origin: China
- name: Race Car 🏎️
  type: Toy Car
  color: Red
  age_range: 3-5
  price: 25
  manufacturer: Mattel
  in_stock: true
  weight: 0.75
  dimensions: "8 x 4 x 3"
  safety_certified: true
  country_of_origin: USA
- name: Doll 🎎
  type: Doll
  color: null
  age_range: 3-5
  price: 30
  manufacturer: Mattel
  in_stock: false
  weight: 0.5
  dimensions: "12 x 4 x 2"
  safety_certified: true
  country_of_origin: China
"""
    return data


@pytest.fixture
def toys_yaml():
    return yaml.safe_load(toys_yaml_str())
