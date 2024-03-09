/*
Prueba de matrices (arrays) 
*/
fn main() {

    // Las matrices son listas de elementos del mismo tipo
    // Tienen una longitud fijada en tiempo de compilación
    let matriz1 = [5,9,2,4];

    // El primer elemento de la matriz tiene índice 0
    println!("El primer elemento de matriz1 es {}, y el segundo es {}", matriz1[0], matriz1[1]);

    // Matriz iniciada con el mismo valor para todos los elementos
    let mmonotona = [ 33; 5];

    // Esta matriz tiene el mismo número en todas las posiciones
    println!("Primer elemento: {}\nSegundo elemento: {}\nTercer elemento: {}", mmonotona[0], mmonotona[1], mmonotona[2]);
}
