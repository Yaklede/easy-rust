pub(crate) fn pointer() {
    //they are not same type , they refer different stack memory, so you can't compare for types
    let my_number = 10;
    let single_reference = &my_number;
    let double_reference = &single_reference;
    let five_reference = &&&&&double_reference;
}

